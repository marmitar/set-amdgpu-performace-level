# set-amdgpu-performace-level

Change `/sys/class/drm/card*/device/power_dpm_force_performance_level`.

## Install

```sh
> cargo +nightly build --release
> sudo cp target/release/set-amdgpu-performace-level /usr/local/bin/set-amdgpu-performace-level
> sudo chown root:root /usr/local/bin/set-amdgpu-performace-level
> sudo chmod 0755 /usr/local/bin/set-amdgpu-performace-level
> sudo setcap cap_sys_admin,cap_dac_override+ep /usr/local/bin/set-amdgpu-performace-level
```

Alternatively, just use the [`install`](install) script.
