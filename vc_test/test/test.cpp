
#include <iostream>

#include <libreqwest.h>

int main()
{
    auto handle = reqwest_create(1);

    auto key = reqwest_url(handle, "https://www.google.com");

    uint8_t status;
    while (status = reqwest_check(handle, key) == 0)
    {
        std::cout << "wait finish key:" << key << std::endl;
    }

    std::cout <<"key: "<< key <<"status: "<<(int)status << std::endl;

    uint32_t len = 0;
    auto data= reqwest_get_data(handle, key, &len);
    
    auto str= std::string((const char*)data,(size_t)len);

    std::cout << str << std::endl;

    reqwest_finish_url(handle, key);

 
    std::cout << "finish" << std::endl;

    std::getchar();
}

