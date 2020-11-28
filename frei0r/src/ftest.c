/*
gcc -o ftest ftest.c -ldl
 */
#include <dlfcn.h>
#include <frei0r.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    void *handle;
    char *error;

    char* path = "/usr/lib/frei0r-1/saturat0r.so";
    printf("loading %s\n", path);
    handle = dlopen(path, RTLD_LAZY);
    if (!handle) {
        fputs (dlerror(), stderr);
        exit(1);
    }

    printf("f0r_init\n");
    void (*f0r_init)();
    f0r_init = dlsym(handle, "f0r_init");
    if ((error = dlerror()) != NULL)  {
        fputs(error, stderr);
        exit(1);
    }
    f0r_init();

    struct f0r_plugin_info info;

    printf("f0r_get_plugin_info\n");
    void (*f0r_get_plugin_info)(f0r_plugin_info_t*);
    f0r_get_plugin_info = dlsym(handle, "f0r_get_plugin_info");
    if ((error = dlerror()) != NULL)  {
        fputs(error, stderr);
        exit(1);
    }
    f0r_get_plugin_info(&info);

    printf("'%s' by '%s'\n", info.name, info.author);
    printf("'%s'\n", info.explanation);
    printf("plugin type %d\n", info.plugin_type);
    printf("num params %d, color model %d\n", info.num_params, info.color_model);
    printf("version %d %d %d\n", info.frei0r_version, info.major_version, info.minor_version);

    printf("f0r_construct\n");
    f0r_instance_t (*f0r_construct)(unsigned int width, unsigned int height);
    f0r_construct = dlsym(handle, "f0r_construct");
    if ((error = dlerror()) != NULL)  {
        fputs(error, stderr);
        exit(1);
    }

    unsigned int width = 8;
    unsigned int height = 8;
    f0r_instance_t f0r_instance = f0r_construct(width, height);

    printf("f0r_set_param_value\n");
    void (*f0r_set_param_value)(f0r_instance_t*, f0r_param_t, int);
    f0r_set_param_value = dlsym(handle, "f0r_set_param_value");
    if ((error = dlerror()) != NULL)  {
        fputs(error, stderr);
        exit(1);
    }
    double saturation = 0.1;
    f0r_set_param_value(f0r_instance, (f0r_param_t)&saturation, 0);

    uint32_t inframe[width * height];
    for (size_t i = 0; i < width * height; ++i) {
        inframe[i] = i;
    }
    for (size_t y = 0; y < height; ++y) {
        for (size_t x = 0; x < width; ++x) {
            size_t ind = y * width + x;
            printf("%d ", inframe[ind]);
        }
        printf("\n");
    }
    printf("update\n");
    uint32_t outframe[width * height];

    printf("f0r_update\n");
    void (*f0r_update)(f0r_instance_t*, double, const uint32_t*, uint32_t*);
    f0r_update = dlsym(handle, "f0r_update");
    if ((error = dlerror()) != NULL)  {
        fputs(error, stderr);
        exit(1);
    }
    f0r_update(f0r_instance, 0.0, inframe, outframe);
    for (size_t y = 0; y < height; ++y) {
        for (size_t x = 0; x < width; ++x) {
            size_t ind = y * width + x;
            printf("%d ", outframe[ind]);
        }
        printf("\n");
    }
    dlclose(handle);
    printf("done with frei0r\n");
}
