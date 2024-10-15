use std::env;

use smithay_client_toolkit::{
    delegate_output, delegate_registry,
    output::OutputState,
    reexports::{
        client::{
            globals::{registry_queue_init, Global, GlobalListContents},
            protocol::{
                wl_compositor::WlCompositor,
                wl_output::{self, WlOutput},
                wl_registry,
            },
            Connection, Dispatch, QueueHandle,
        },
        protocols::wp::fractional_scale::v1::client::{
            wp_fractional_scale_manager_v1::{self, WpFractionalScaleManagerV1},
            wp_fractional_scale_v1,
        },
    },
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
};

struct MyApp {
    exit: bool,
}

struct MyUserData;

// You need to provide a Dispatch<WlRegistry, GlobalListContents> impl for your app
impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for MyApp {
    fn event(
        state: &mut MyApp,
        proxy: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        // This mutex contains an up-to-date list of the currently known globals
        // including the one that was just added or destroyed
        data: &GlobalListContents,
        conn: &Connection,
        qhandle: &QueueHandle<MyApp>,
    ) {
        /* react to dynamic global events here */
    }
}

impl Dispatch<WpFractionalScaleManagerV1, MyUserData> for MyApp {
    fn event(
        state: &mut Self,
        proxy: &WpFractionalScaleManagerV1,
        event: wp_fractional_scale_manager_v1::Event,
        data: &MyUserData,
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        todo!()
    }
}

impl Dispatch<WlOutput, MyUserData> for MyApp {
    fn event(
        state: &mut Self,
        proxy: &WlOutput,
        event: wl_output::Event,
        data: &MyUserData,
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        if let wl_output::Event::Geometry {
            x,
            y,
            physical_width,
            physical_height,
            subpixel,
            make,
            model,
            transform,
        } = event
        {
            println!(
                "make:{},model:{},x:{},y:{},width:{},height:{}",
                make, model, x, y, physical_width, physical_height
            );
        }
    }
}

fn main() {
    //1.设置环境变量,你决定要连接到哪个wayland compositor
    env::set_var("WAYLAND_DISPLAY", "wayland-0");

    //2.连接到wayland服务器
    let conn = Connection::connect_to_env().expect("connect failed");

    //这个方法会获取wl_display,然后发送get_registry请求,然后获取所有的全局接口
    let (glist, mut event_queue) = registry_queue_init::<MyApp>(&conn).unwrap();

    for ele in glist.contents().clone_list() {
        // println!("{},{},{}", ele.name, ele.interface, ele.version);
    }
    // 绑定全局对象
    let scalManager: WpFractionalScaleManagerV1 = glist
        .bind(&event_queue.handle(), 1..=1, MyUserData)
        .unwrap();
    // 绑定全局对象wl_output
    let wl_output: WlOutput = glist
        .bind(&event_queue.handle(), 1..=1, MyUserData)
        .unwrap();

    let mut my_app = MyApp { exit: false };
    while !my_app.exit {
        event_queue.blocking_dispatch(&mut my_app);
    }
}
