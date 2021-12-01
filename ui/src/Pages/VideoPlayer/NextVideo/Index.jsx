import "./Index.scss";

function NextVideo(props) {
  const { id } = props;

  return (
    <div className="nextVideoOverlay">
      <div className="nextVideoBtn">
        Next Episode
      </div>
    </div>
  );
}

export default NextVideo;
