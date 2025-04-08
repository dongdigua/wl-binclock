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
        protocols_wlr::layer_shell::v1::client::{
            zwlr_layer_shell_v1::{self, ZwlrLayerShellV1},
            zwlr_layer_surface_v1::{self, Anchor, ZwlrLayerSurfaceV1},
        },
    },
    shm::{Shm, ShmHandler},
};
use clap::Parser;
use nix::poll::{poll, PollFd, PollFlags};
use std::time::{Duration, Instant};

mod draw;
mod args;
mod log;

struct MyApp {
    wl_surface: WlSurface,
    shm: Shm,
    width: u32,
    height: u32,
}

impl MyApp {
    const WIDTH: u32 = 16 * 6;
    const HEIGHT: u32 = 16 * 4;
    const PIXEL_SIZE: u32 = 4;
    const STORE_SIZE: u32 = Self::WIDTH * Self::HEIGHT * 2 * Self::PIXEL_SIZE;

    fn new(wl_surface: WlSurface, shm: Shm) -> Self {
        MyApp {
            wl_surface,
            shm,
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
                debug!("wl_surface::Event::Enter");
            }
            wl_surface::Event::Leave { output: _ } => {
                debug!("wl_surface::Event::Leave");
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


impl Dispatch<WlCallback, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &WlCallback,
        _event: wl_callback::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
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
                debug!("layer shell size:{},{}", width, height);
                _proxy.ack_configure(serial);
                state.width = width;
                state.height = height;
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
    let args = args::Args::parse();
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

    //给他layer shell的角色 一个surface只能有一个角色
    let layer_shell: ZwlrLayerShellV1 = glist
        .bind(&event_queue.handle(), 1..=5, MyUserData)
        .unwrap();
    let lay_surface = layer_shell.get_layer_surface(
        &wl_surface,
        None,
        zwlr_layer_shell_v1::Layer::Bottom,
        String::from("wl-binclock"),
        &event_queue.handle(),
        MyUserData,
    );
    lay_surface.set_size(MyApp::WIDTH, MyApp::HEIGHT);
    lay_surface.set_anchor(Anchor::from_bits(args.anchor)
                           .expect("bad anchor"));
    wl_surface.commit();
    //获得wl_shm全局对象
    let shm = Shm::bind(&glist, &event_queue.handle()).unwrap();

    let mut my_app = MyApp::new(wl_surface, shm);
    // CONFIG 0xAARRGGBB
    // let my_painter = draw::Painter::new(draw::Color::Multi(vec![0x80e8b6, 0xa1fff9, 0xbd7cf8, 0x7288f6]), draw::Color::Mono(0xffffff));
    let my_painter = draw::Painter::new(draw::Color::from(args.fg), draw::Color::from(args.bg));

    const ONE_SEC: Duration = Duration::from_secs(1);
    let mut last_update = Instant::now() - ONE_SEC;
    loop {
        // https://docs.rs/wayland-client/latest/wayland_client/struct.EventQueue.html#integrating-the-event-queue-with-other-sources-of-events
        event_queue.flush().unwrap();
        let read_guard = event_queue.prepare_read().unwrap();
        let wl_fd = read_guard.connection_fd();

        let elapsed = last_update.elapsed();
        const MIN_DELAY: u16 = 1;
        let timeout_ms = if elapsed >= ONE_SEC {
            MIN_DELAY
        } else {
            let mut diff = (ONE_SEC - elapsed).as_millis() as u16;
            if diff == 0 { diff += MIN_DELAY; }
            diff
        };

        // Wait for events or timeout.
        let mut poll_fds = [PollFd::new(wl_fd, PollFlags::POLLIN)];

        let poll_ret = poll(&mut poll_fds, timeout_ms).unwrap();
        if  poll_ret > 0 {
            debug!("poll > 0");
            read_guard.read().unwrap();
            event_queue.dispatch_pending(&mut my_app).unwrap();
        } else if poll_ret == 0 {
            debug!("poll timed out in {timeout_ms}");
            std::mem::drop(read_guard);
        } else {
            eprintln!("poll failed");
        }

        if elapsed >= ONE_SEC {
            debug!("update");
            let buffer = my_painter.draw(&my_app);
            buffer.attach_to(&my_app.wl_surface).unwrap();
            my_app.wl_surface.damage(0, 0, i32::MAX, i32::MAX);
            my_app.wl_surface.commit();
            last_update += ONE_SEC;
        }
    }
}
