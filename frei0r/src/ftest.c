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

    dlclose(handle);
    printf("done with frei0r\n");
}
