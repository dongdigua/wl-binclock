use std::{env, ffi::CString, i32, os::fd::AsFd};

use nix::sys::memfd;
use smithay_client_toolkit::reexports::{
    client::{
        globals::{registry_queue_init, GlobalListContents},
        protocol::{
            wl_buffer::{self, WlBuffer},
            wl_compositor::{self, WlCompositor},
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
};

struct MyApp {
    exit: bool,
}

struct MyUserData;

const WIDTH: i32 = 600;
const HEIGHT: i32 = 400;
const STRIDE: i32 = WIDTH * 4;
const STORE_SIZE: i32 = WIDTH * HEIGHT * 2 * 4;

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
    }
}

impl Dispatch<WlShm, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &WlShm,
        _event: wl_shm::Event,
        _data: &MyUserData,
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
        _proxy: &XdgWmBase,
        _event: xdg_wm_base::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<XdgSurface, MyUserData> for MyApp {
    fn event(
        _state: &mut Self,
        _proxy: &XdgSurface,
        _event: xdg_surface::Event,
        _data: &MyUserData,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
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

    for ele in glist.contents().clone_list() {
        println!("{},{},{}", ele.name, ele.interface, ele.version);
    }
    //绑定到全局对象wl_compositor
    let wl_compositor: WlCompositor = glist
        .bind(&event_queue.handle(), 1..=6, MyUserData)
        .unwrap();

    //申请一张纸
    let wl_surface = wl_compositor.create_surface(&event_queue.handle(), MyUserData);

    //把这张纸转换成xdg_surface,这样才能在常见的桌面环境下显示
    let xdg_wm_base: XdgWmBase = glist
        .bind(&event_queue.handle(), 1..=6, MyUserData)
        .unwrap();
    let xdg_surface = xdg_wm_base.get_xdg_surface(&wl_surface, &event_queue.handle(), MyUserData);
    //这一行必须要不然没有窗口显示
    let xdg_toplevel = xdg_surface.get_toplevel(&event_queue.handle(), MyUserData);
    xdg_toplevel.set_title(String::from("test"));

    //获得wl_shm全局对象
    let wl_shm: WlShm = glist
        .bind(&event_queue.handle(), 1..=1, MyUserData)
        .unwrap();

    //申请一段共享内存
    let fd = memfd::memfd_create(
        CString::new("shm_file").unwrap().as_c_str(),
        memfd::MemFdCreateFlag::empty(),
    )
    .unwrap();
    //为共享内存分配大小
    nix::unistd::ftruncate(fd.try_clone().unwrap(), STORE_SIZE.into()).unwrap();

    //通知Server端，我现在要用这个文件放像素(Server端也同样会做内存映射)
    let wl_shm_pool = wl_shm.create_pool(fd.as_fd(), STORE_SIZE, &event_queue.handle(), MyUserData);
    //通知Server端,我要现在要在上面存一个buffer,同时告诉他buffer的大小,存在里面字节的像素格式
    //xrgb8888一共是32位,也就是4字节，一个像素就是4字节,注意这个STRIDE值
    let wl_buffer = wl_shm_pool.create_buffer(
        0,
        WIDTH,
        HEIGHT,
        STRIDE,
        wl_shm::Format::Xrgb8888,
        &event_queue.handle(),
        MyUserData,
    );
    //映射这段共享内存到客户端
    let mut mmap = unsafe {
        memmap2::MmapOptions::new()
            .len(STORE_SIZE as usize)
            .map_mut(&fd)
            .unwrap()
    };
    //开始画画
    //这个内存映射本身就相当与一个8位为一个字节的数组
    //但是我们的字节是32位的,所以我们要把这8位字节的对齐到32位。
    //如果这个字节数组不是32位的整数倍,pre和end就会多出来几个字节。
    let (_pre, middle, _end) = unsafe { mmap.align_to_mut::<u32>() };
    middle.fill(0xFF00FFFF);

    //开始倾倒：把这桶buffer油漆放到这张surface纸上
    wl_surface.attach(Some(&wl_buffer), 0, 0);
    //告诉服务端这张纸需要更新
    wl_surface.damage(0, 0, i32::MAX, i32::MAX);
    //告诉Server端倾倒完成
    wl_surface.commit();

    let mut my_app = MyApp { exit: false };
    //利用wl_compistor创建一个wl_surface
    while !my_app.exit {
        event_queue.blocking_dispatch(&mut my_app).unwrap();
    }
}
