use std::{env, i32};

use smithay_client_toolkit::{
    globals::GlobalData,
    reexports::{
        client::{
            globals::{registry_queue_init, GlobalListContents},
            protocol::{
                wl_buffer::{self, WlBuffer},
                wl_callback::{self, WlCallback},
                wl_compositor::{self, WlCompositor},
                wl_output::{self, WlOutput},
                wl_registry,
                wl_shm::{self, WlShm},
                wl_shm_pool::{self, WlShmPool},
                wl_surface::{self, WlSurface},
            },
            Connection, Dispatch, QueueHandle,
        },
        protocols::xdg::shell::client::{
            xdg_surface::{self, XdgSurface},
            xdg_toplevel::{self, XdgToplevel},
            xdg_wm_base::{self, XdgWmBase},
        },
        protocols_wlr::layer_shell::v1::client::{
            zwlr_layer_shell_v1::{self, ZwlrLayerShellV1},
            zwlr_layer_surface_v1::{self, Anchor, ZwlrLayerSurfaceV1},
        },
    },
    shm::{Shm, ShmHandler},
};
use water_bg_config::Config;

mod draw;

struct MyApp {
    exit: bool,
    wl_surface: WlSurface,
    shm: Shm,
    has_draw: bool,
    width: u32,
    height: u32,
    config: Config,
}

impl MyApp {
    const PIXEL_SIZE: u32 = 4;

    fn new(wl_surface: WlSurface, shm: Shm) -> Self {
        MyApp {
            exit: false,
            wl_surface,
            shm,
            has_draw: false,
            width: 0,
            height: 0,
            config: Config::default(),
        }
    }

    fn store_size(&self) -> usize {
        (self.width * self.height * 2 * Self::PIXEL_SIZE) as usize
    }

    fn stride(&self) -> i32 {
        (self.width * Self::PIXEL_SIZE) as i32
    }
}

struct MyUserData;

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for MyApp {
    fn event(
        _state: &mut MyApp,
        _proxy: &wl_registry::WlRegistry,
        _event: wl_registry::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        _qhandle: &QueueHandle<MyApp>,
    ) {
    }
}

impl Dispatch<WlCompositor, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &WlCompositor,
        _event: wl_compositor::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlSurface, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &WlSurface,
        _event: wl_surface::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match _event {
            wl_surface::Event::Enter { output: _ } => {
                println!("wl_surface::Event::Enter")
            }
            wl_surface::Event::Leave { output: _ } => {
                println!("wl_surface::Event::Leave")
            }
            _ => (),
        }
    }
}

impl Dispatch<WlShm, GlobalData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &WlShm,
        _event: wl_shm::Event,
        _data: &GlobalData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlShmPool, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &WlShmPool,
        _event: wl_shm_pool::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlBuffer, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &WlBuffer,
        _event: wl_buffer::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<XdgWmBase, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        proxy: &XdgWmBase,
        event: xdg_wm_base::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            xdg_wm_base::Event::Ping { serial } => {
                proxy.pong(serial);
            }
            _ => (),
        }
    }
}

impl Dispatch<XdgSurface, MyUserData> for MyApp {
    fn event(
        state: &mut Self,
        proxy: &XdgSurface,
        event: xdg_surface::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match event {
            xdg_surface::Event::Configure { serial } => {
                if state.has_draw {
                    return;
                }
                proxy.ack_configure(serial);
                let buffer = draw::Painter::draw(&state);
                buffer.attach_to(&state.wl_surface).unwrap();
                state.wl_surface.damage(0, 0, i32::MAX, i32::MAX);
                state.wl_surface.commit();
                state.has_draw = true;
            }
            _ => (),
        }
    }
}

impl Dispatch<XdgToplevel, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &XdgToplevel,
        _event: xdg_toplevel::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        println!("XdgToplevel:{:?}", _event);
    }
}

impl Dispatch<WlCallback, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &WlCallback,
        _event: wl_callback::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match _event {
            wl_callback::Event::Done { callback_data: _ } => {}
            _ => (),
        }
    }
}

impl Dispatch<WlOutput, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &WlOutput,
        _event: wl_output::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match _event {
            wl_output::Event::Scale { factor: _ } => {
                println!("scale factor:{:?}", _event);
            }
            _ => (),
        }
    }
}

impl Dispatch<ZwlrLayerShellV1, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &ZwlrLayerShellV1,
        _event: zwlr_layer_shell_v1::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, MyUserData> for MyApp {
    fn event(
        state: &mut Self,
        _proxy: &ZwlrLayerSurfaceV1,
        _event: zwlr_layer_surface_v1::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        match _event {
            zwlr_layer_surface_v1::Event::Configure {
                serial,
                width,
                height,
            } => {
                println!("layer shell size:{},{}", width, height);
                if state.has_draw {
                    return;
                }
                _proxy.ack_configure(serial);
                state.width = width;
                state.height = height;

                let buffer = draw::Painter::draw(&state);
                buffer.attach_to(&state.wl_surface).unwrap();
                state.wl_surface.damage(0, 0, i32::MAX, i32::MAX);
                state.wl_surface.commit();
                state.has_draw = true;
            }
            zwlr_layer_surface_v1::Event::Closed => {}
            _ => (),
        }
    }
}

impl ShmHandler for MyApp {
    fn shm_state(&mut self) -> &mut Shm {
        todo!()
    }
}

fn main() {
    env::set_var("WAYLAND_DISPLAY", "wayland-0");
    let conn = Connection::connect_to_env().expect("connect failed");
    let (glist, mut event_queue) = registry_queue_init::<MyApp>(&conn).unwrap();
    let wl_compositor: WlCompositor = glist
        .bind(&event_queue.handle(), 1..=6, MyUserData)
        .unwrap();
    let wl_surface = wl_compositor.create_surface(&event_queue.handle(), MyUserData);
    let layer_shell: ZwlrLayerShellV1 = glist
        .bind(&event_queue.handle(), 1..=5, MyUserData)
        .unwrap();
    let lay_surface = layer_shell.get_layer_surface(
        &wl_surface,
        None,
        zwlr_layer_shell_v1::Layer::Background,
        String::new(),
        &event_queue.handle(),
        MyUserData,
    );
    lay_surface.set_anchor(Anchor::all());
    lay_surface.set_exclusive_zone(-1);
    wl_surface.commit();
    let shm = Shm::bind(&glist, &event_queue.handle()).unwrap();

    let mut my_app = MyApp::new(wl_surface, shm);

    while !my_app.exit {
        event_queue.blocking_dispatch(&mut my_app).unwrap();
    }
}
