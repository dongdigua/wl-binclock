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

mod draw;
mod wallpaper;

struct MyApp {
    exit: bool,
    wl_surface: WlSurface,
    shm: Shm,
    has_draw: bool,
    width: u32,
    height: u32,
}

impl MyApp {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 400;
    const PIXEL_SIZE: u32 = 4;
    const STORE_SIZE: u32 = Self::WIDTH * Self::HEIGHT * 2 * Self::PIXEL_SIZE;

    fn new(wl_surface: WlSurface, shm: Shm) -> Self {
        MyApp {
            exit: false,
            wl_surface,
            shm,
            has_draw: false,
            width: Self::WIDTH,
            height: Self::HEIGHT,
        }
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
                //开始画画
                let buffer = draw::Painter::draw(&state);
                //开始倾倒：把这桶buffer油漆放到这张surface纸上
                buffer.attach_to(&state.wl_surface).unwrap();
                //告诉服务端这张纸需要更新
                state.wl_surface.damage(0, 0, i32::MAX, i32::MAX);
                //告诉Server端倾倒完成
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

                //开始画画
                let buffer = draw::Painter::draw(&state);
                //开始倾倒：把这桶buffer油漆放到这张surface纸上
                buffer.attach_to(&state.wl_surface).unwrap();
                //告诉服务端这张纸需要更新
                state.wl_surface.damage(0, 0, i32::MAX, i32::MAX);
                //告诉Server端倾倒完成
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
    //设置环境变量,你决定要连接到哪个wayland compositor
    //默认是wayland-0 .具体文件放在/run/user/1000/下
    env::set_var("WAYLAND_DISPLAY", "wayland-0");

    //连接到wayland服务器
    let conn = Connection::connect_to_env().expect("connect failed");

    //这个方法会获取wl_display,然后发送get_registry请求,然后获取所有的全局接口
    let (glist, mut event_queue) = registry_queue_init::<MyApp>(&conn).unwrap();

    //绑定到全局对象wl_compositor
    let wl_compositor: WlCompositor = glist
        .bind(&event_queue.handle(), 1..=6, MyUserData)
        .unwrap();

    //申请一张纸
    //刚刚创建的时候，他是初始状态,初始状态是无效的。
    let wl_surface = wl_compositor.create_surface(&event_queue.handle(), MyUserData);
    // wl_surface.frame(&event_queue.handle(), MyUserData);
    //把这张纸转换成xdg_surface,这样才能在常见的桌面环境下显示
    // let xdg_wm_base: XdgWmBase = glist
    //     .bind(&event_queue.handle(), 1..=6, MyUserData)
    //     .unwrap();
    // let xdg_surface = xdg_wm_base.get_xdg_surface(&wl_surface, &event_queue.handle(), MyUserData);

    //为他分配一个角色，让他显示到最上层,这样他就不是初始状态了。
    // let xdg_toplevel = xdg_surface.get_toplevel(&event_queue.handle(), MyUserData);
    // xdg_toplevel.set_title(String::from("test"));

    //给他layer shell的角色 一个surface只能有一个角色
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
    println!("第一次wl_surface提交");
    //获得wl_shm全局对象
    let shm = Shm::bind(&glist, &event_queue.handle()).unwrap();

    let mut my_app = MyApp::new(wl_surface, shm);
    //利用wl_compistor创建一个wl_surface
    while !my_app.exit {
        event_queue.blocking_dispatch(&mut my_app).unwrap();
    }
}
