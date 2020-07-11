# rust-actix-react-web-starter
A bare Rust web app that uses [actix-web](https://actix.rs/), [Diesel](http://diesel.rs/) and [React](https://reactjs.org/) to get you started building your Rust and React applications.

## Usage

```bash
git clone https://github.com/16kilobyte/rust-actix-react-web-starter.git
cd  rust-actix-react-web-starter/api_server
cp .env.sample .env
diesel setup --database-url='postgres://user:password@localhost:5432/api_server_db'
diesel migration run
cargo run
# Started http server: 127.0.0.1:3000
```

### web client

[http://127.0.0.1:3000/](http://127.0.0.1:3000/)

## License
This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/16kilobyte/rust-actix-react-web-starter/blob/master/LICENSE) file for details.
