#[macro_use]
mod dispatch_thread_pool;
mod mandelbrot;
mod web_server;
mod webserver_threadpool;

fn main() {
    //mandelbrot::test_madelbrot();
    //dispatch_thread_pool::dispatch_thread_pool_run();
    web_server::start_listener();
}
