#!/bin/bash
echo "host all  all    127.0.0.1/0  md5" > /var/lib/postgresql/10/main/pg_hba.conf
echo "listen_addresses='*'" > /var/lib/postgresql/10/main/postgresql.conf
sudo -u postgres /usr/lib/postgresql/10/bin/pg_ctl -D /var/lib/postgresql/10/main start 

if [[ $DIM_ENABLE_SSL == "enable" ]];
then
    cd /opt/dim && ./dim --priv-key /opt/dim/config/priv_key.pem --ssl-cert /opt/dim/config/cert.pem;
else
    cd /opt/dim && ./dim;
fi
