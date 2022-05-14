const createProxyMiddleware = require("http-proxy-middleware");

module.exports = function (app) {
  app.use(
    "/api",
    createProxyMiddleware({
      target: "http://127.0.0.1:8000",
      changeOrigin: true,
    })
  );

  app.use(
    "/images",
    createProxyMiddleware({
      target: "http://127.0.0.1:8000",
      changeOrigin: true,
    })
  );

  app.use(
    "/ws",
    createProxyMiddleware({
      target: "http://127.0.0.1:8000",
      ws: true,
      changeOrigin: true,
    })
  );
};
