#!/bin/bash
if [[ $DIM_ENABLE_SSL == "1" ]];
then
    cd /opt/dim && ./dim --priv-key /opt/dim/config/priv_key.pem --ssl-cert /opt/dim/config/cert.pem --cache-dir /opt/dim/transcoding;
else
    cd /opt/dim && ./dim --cache-dir /opt/dim/transcoding;
fi
