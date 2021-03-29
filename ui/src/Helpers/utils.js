// converts to HH:MM:SS format
export const formatHHMMSS = (secs) => (
  new Date(secs * 1000).toISOString().substr(11, 8)
);
