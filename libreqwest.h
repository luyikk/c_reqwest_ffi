
struct ReqwestHandle;

extern "C" {

/// 新建运行时
/// thread_count 表示并发线程数量
ReqwestHandle *reqwest_create(uint32_t thread_count);

/// 释放运行时
void reqwest_release(ReqwestHandle *handler);

/// 开始get url
unsigned long long reqwest_url(ReqwestHandle *handler, const char *url);

/// 检查是否完成 0=没有完成 1=成功 2=错误
unsigned char reqwest_check(ReqwestHandle *handler, unsigned long long key);

/// 获取url 数据 check=1 表示数据 check=2表示错误信息
const unsigned char *reqwest_get_data(ReqwestHandle *handler,
                                      unsigned long long key,
                                      unsigned int *len);

/// 完成url
void reqwest_finish_url(ReqwestHandle *handler, unsigned long long key);

} // extern "C"
