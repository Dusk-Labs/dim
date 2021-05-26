// converts to HH:MM:SS format
export const formatHHMMSS = (secs) => (
  new Date(secs * 1000).toISOString().substr(11, 8)
);

/*
  maintains the ratio when finding
  new width/height of an element
*/
export const calcNewSize = (
  currentWidth,
  currentHeight,
  newWidth = undefined,
  newHeight = undefined
) => {
  const ratio = currentWidth / currentHeight;

  if (newWidth !== undefined) {
    return Math.round(newWidth / ratio);
  }

  if (newWidth !== undefined) {
    return Math.round(newHeight * ratio);
  }
};
