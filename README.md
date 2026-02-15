# Virtual Display Daemon for Fedora

[![Copr build status](https://copr.fedorainfracloud.org/coprs/pvermeer/virtual-display/package/virtual-display/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/pvermeer/virtual-display/package/virtual-display/)

`virtual-display` Is a Daemon and CLI tool for fedora to enable a virtual display on an empty connector. This leverages the `kernel debug sys paths` to set an EDID on an empty connector.

It is available on [copr](https://copr.fedorainfracloud.org/coprs/pvermeer/virtual-display) or you can build from source (cargo build).

Also it's experimental and **might break on updates!**

## There must be an empty connector available on the system

To display connectors info run:

```sh
for connector_path in /sys/class/drm/*/status; do
    if [[ "$(cat "$connector_path")" == "unknown" ]]; then continue; fi
    status=${connector_path%/status}
    connector_name="${status#*/card?-}: "
    echo -n " $connector_name"
    cat "$connector_path"
done
```

Or run `virtual-display status` after install.

This should display the following info for your system. If there are no disconnected connectors, this tool is not going to work.

```sh
 DP-1: connected
 DP-2: disconnected
 DP-3: disconnected
 HDMI-A-1: connected
```

```
Usage: virtual-display <COMMAND>

  status   Get status
    --json  Print json

  daemon   Run daemon commands
    start  Start the daemon (systemd)
    stop   Stop the daemon (systemd)

  enable   Enable virtual display
    -c, --connector <CONNECTOR>  Name of the display connector

  disable  Disable virtual display
```

## Example implementation

Check my [before-stream.sh](https://github.com/PVermeer/dotfiles/blob/main/.config/sunshine/before-stream.sh) script that uses this tool to stream from a 4k virtual display to my tv with [Sunshine](https://github.com/LizardByte/Sunshine).

### Fedora copr

https://copr.fedorainfracloud.org/coprs/pvermeer/virtual-display
