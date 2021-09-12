function cat_device {
    cat /dev/serial/by-id/usb-Teensyduino_USB_Serial_9986080-if00
}

while true
do
    cat_device
    echo 'Connection broken, waiting for serial...'
    sleep 5
done