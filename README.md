# Hackable.se


## API


Dev:
```
apt-get install libmysqlclient-dev mysql-server
sudo mysql
> CREATE DATABASE hackable_se;
> GRANT ALL ON hackable_se.* TO hackable_se@localhost IDENTIFIED BY '...';

echo "DATABASE_URL=mysql://hackable_se:aXjLmRO9zAJx1e26@localhost/hackable_se" > .env
```



## Scrapbook

```
curl http://localhost:8000/users/123e4567-e89b-12d3-a456-426655440000
curl http://localhost:8000/users/31b7ff47-a899-42d6-881c-ac936064bfae  
curl http://localhost:8000/users --data '{"username": "ZetaTwo", "password": "123456", "email": "calle.svensson@zeta-two.com"}' -H 'Content-Type: application/json'
```
