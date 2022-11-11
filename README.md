# rust systemd parser

An example without symlink:

```
root@main:~# ls -l /usr/lib/systemd/system/thermald.service
-rw-r--r-- 1 root root 309 Aug 31  2021 /usr/lib/systemd/system/thermald.service
root@main:~# ./rust-systemd-parser /usr/lib/systemd/system/thermald.service
{
    "Install": [
        {
            "WantedBy": "multi-user.target",
        },
        {
            "Alias": "dbus-org.freedesktop.thermald.service",
        },
    ],
    "Unit": [
        {
            "Description": "Thermal Daemon Service",
        },
        {
            "ConditionVirtualization": "no",
        },
    ],
    "Service": [
        {
            "Type": "dbus",
        },
        {
            "SuccessExitStatus": "2",
        },
        {
            "BusName": "org.freedesktop.thermald",
        },
        {
            "ExecStart": "/usr/sbin/thermald --systemd --dbus-enable --adaptive",
        },
        {
            "Restart": "on-failure",
        },
    ],
}
```

Another example with symlink:

```
root@main:~# ls -l /usr/lib/systemd/system/udev.service
lrwxrwxrwx 1 root root 21 Jan 10  2022 /usr/lib/systemd/system/udev.service -> systemd-udevd.service
root@main:~# ./rust-systemd-parser /usr/lib/systemd/system/udev.service
{
    "Service": [
        {
            "DeviceAllow": "block-* rwm",
        },
        {
            "DeviceAllow": "char-* rwm",
        },
        {
            "Type": "notify",
        },
        {
            "OOMScoreAdjust": "-1000",
        },
        {
            "Sockets": "systemd-udevd-control.socket systemd-udevd-kernel.socket",
        },
        {
            "Restart": "always",
        },
        {
            "RestartSec": "0",
        },
        {
            "ExecStart": "/lib/systemd/systemd-udevd",
        },
        {
            "ExecReload": "udevadm control --reload --timeout 0",
        },
        {
            "KillMode": "mixed",
        },
        {
            "TasksMax": "infinity",
        },
        {
            "PrivateMounts": "yes",
        },
        {
            "ProtectClock": "yes",
        },
        {
            "ProtectHostname": "yes",
        },
        {
            "MemoryDenyWriteExecute": "yes",
        },
        {
            "RestrictAddressFamilies": "AF_UNIX AF_NETLINK AF_INET AF_INET6",
        },
        {
            "RestrictRealtime": "yes",
        },
        {
            "RestrictSUIDSGID": "yes",
        },
        {
            "LockPersonality": "yes",
        },
        {
            "IPAddressDeny": "any",
        },
        {
            "WatchdogSec": "3min",
        },
    ],
    "Unit": [
        {
            "Description": "Rule-based Manager for Device Events and Files",
        },
        {
            "Documentation": "man:systemd-udevd.service(8) man:udev(7)",
        },
        {
            "DefaultDependencies": "no",
        },
        {
            "After": "systemd-sysusers.service systemd-hwdb-update.service",
        },
        {
            "Before": "sysinit.target",
        },
        {
            "ConditionPathIsReadWrite": "/sys",
        },
    ],
}
```

