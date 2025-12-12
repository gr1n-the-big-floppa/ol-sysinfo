# ol-sysinfo

**ol-sysinfo** is an [OmniLED](https://github.com/llMBQll/OmniLED) plugin that collects and sends hardware information such as CPU usage, memory statistics, and temperature sensor readings.

## Running

`ol-sysinfo` expects two arguments: the server address and the update interval (in seconds):

```shell
ol-sysinfo --address <ADDRESS> --update-interval <UPDATE_INTERVAL>
```

## Events

The plugin emits a `SYSINFO` event every `<UPDATE_INTERVAL>` seconds.
Each event contains:

* CPU usage
* Memory statistics
* Temperature readings for all detected sensors

### `SYSINFO` event structure (table)

* `cpu usage`: float
* `used memory`: float
* `total memory`: float
* `free memory`: float
* `available memory`: float
* `<TEMP_SENSOR_NAME_1>`: float
* ...
* `<TEMP_SENSOR_NAME_N>`: float

## Example configuration

### `applications.lua`

```lua
load_app {
    path = get_default_path('ol-sysinfo'),
    args = {
        '--address', SERVER.Address,
        '--update-interval', 1
    }
}
```

### `scripts.lua`

```lua
function round(num)
    return num >= 0 and math.floor(num + 0.5) or math.ceil(num - 0.5)
end

local function temps()
    local cpu_temp = round(math.max(SYSINFO["k10temp Tccd1"], SYSINFO["k10temp Tccd2"]))
    local cpu_temp_str = string.format("CPU: %iC (%i%s)", cpu_temp, round(SYSINFO["cpu usage"]), "%")
    local gpu_str = string.format("GPU: %iC (%iC)", SYSINFO["amdgpu edge"], SYSINFO["amdgpu junction"])
    local mem_str = string.format(
        "MEM: %.0fGB (%.1f%s)",
        SYSINFO["used memory"] / (1024^3),
        SYSINFO["used memory"] / SYSINFO["total memory"] * 100.0,
        "%"
    )
    local text_size = 12

    return {
        widgets = {
            Widget.Text {
                text = gpu_str,
                font_size = FontSize.Value(text_size),
                position = { x = 0, y = 0 },
                size = { width = SCREEN.Width, height = text_size }
            },
            Widget.Text {
                text = cpu_temp_str,
                font_size = FontSize.Value(text_size),
                position = { x = 0, y = text_size },
                size = { width = SCREEN.Width, height = text_size }
            },
            Widget.Text {
                text = mem_str,
                font_size = FontSize.Value(text_size),
                position = { x = 0, y = text_size * 2 },
                size = { width = SCREEN.Width, height = text_size }
            }
        },
        duration = 100,
    }
end

SCREEN_BUILDER
    :new('SteelSeries Apex 7 TKL')
    :with_layout_group({
        {
            layout = temps,
            run_on = { 'SYSINFO' }
        }
    })
    :register()
```

## Building

Edit the path to omni-led-api in ![Cargo.toml](Cargo.toml), then:

```shell
cargo build --release
```
