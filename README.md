# [WIP] godot-bulletml

BulletML GDExtension for Godot

## Usage

### Editor settings

![image](https://github.com/atty303/godot-bulletml/assets/316079/c1257a12-bdb9-4607-b1f9-790b8fbcb7f9)

For scan bulletml file edited in external editor, set `Autoscan Project Path` to path that contains BulletML files.

### Editing BulletML in JetBrains IDE

#### Set BulletML DTD

![image](https://github.com/atty303/godot-bulletml/assets/316079/140600ef-1225-4853-b047-56dd62d8f692)

[File | Settings | Languages & Frameworks | Schemas and DTDs](jetbrains://idea/settings?name=Languages+%26+Frameworks--Schemas+and+DTDs)

Add URL `http://www.asahi-net.or.jp/~cs8k-cyu/bulletml` and DTD file to `bulletml.dtd`.

## Build

```bash
cargo install cargo-post
cargo post build
```

