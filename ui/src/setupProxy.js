const createProxyMiddleware = require("http-proxy-middleware");
const proxyBaseUrl = process.env.PROXY_BASE
  ? process.env.PROXY_BASE
  : "127.0.0.1:8000";

module.exports = function (app) {
  app.use(
    "/api",
    createProxyMiddleware({
      target: `http://${proxyBaseUrl}`,
      changeOrigin: true,
    })
  );

  app.use(
    "/images",
    createProxyMiddleware({
      target: `http://${proxyBaseUrl}`,
      changeOrigin: true,
    })
  );

  app.use(
    "/ws",
    createProxyMiddleware({
      target: `http://${proxyBaseUrl}`,
      ws: true,
      changeOrigin: true,
    })
  );
};
