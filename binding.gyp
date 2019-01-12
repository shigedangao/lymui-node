
{
  "targets": [
    {
      "target_name": "lymuilib",
      "sources": [
        "src/bootstrap.c",
        "src/binding_util.c",
        "src/bridge.c",
        "src/factory.c",
        "src/convert_xyz.c",
        "src/convert_rgb.c",
        "src/convert_regular.c",
        "src/convert_space.c",
        "src/deserializer/deserializer.c",
        "src/deserializer/deserializer_space.c",
        "src/deserializer/deserializer_opts.c",
        "src/formater.c",
        "src/normalizer/normalizer_rgb.c",
        "src/normalizer/normalizer_regular.c",
        "src/normalizer/normalizer_space.c",
        "src/normalizer/normalizer_xyz.c",
      ],
      "include_dirs": [
        "lymui/bin/include",
        "src/include"
      ],
      "libraries": [
        "<(module_root_dir)/lymui/bin/liblymui.a"
      ]
    }
  ]
}
