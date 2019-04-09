
{
  "targets": [
    {
      "target_name": "lymuilib",
      "sources": [
        "src/bootstrap.c",
        "src/binding_util.c",
        "src/bridge.c",
        "src/factory/factory_common.c",
        "src/factory/factory_regular.c",
        "src/factory/factory_space.c",
        "src/factory/factory_operation.c",
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
  ],
  "conditions": [
    ['OS=="win"', {
      "targets": [
        {
          "libraries": [
            "C:/Program Files (x86)/Microsoft Visual Studio/2017/Enterprise/MSBuild/15.0/VC/lib/legacy_stdio_definitions.lib"
          ]
        }
      ]
    }]
  ]
}
