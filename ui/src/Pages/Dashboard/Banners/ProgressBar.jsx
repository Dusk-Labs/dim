import "./ProgressBar.scss";
import CircleIcon from "../../../assets/Icons/Circle";

function ProgressBar(props) {
  const { season, episode } = props.data;
  let { delta, duration } = props.data;

  delta = !delta ? 0 : delta;
  duration = Math.round(duration / 60);

  const current = Math.round(delta / 60);
  const width = (current / duration) * 100 + "%";

  return (
    <div className="bannerProgressBar">
      {(season || season === 0) && (episode || episode === 0) && (
        <div className="s-e">
          <p>S{season}</p>
          <CircleIcon />
          <p>E{episode}</p>
        </div>
      )}
      <div className="progress">
        <div className="current">
          <p>{current}</p>
          <p>min</p>
        </div>
        <div className="bar">
          <span className="progress-fill" style={{ width: width }} />
        </div>
        <div className="duration">
          <p>{duration}</p>
          <p>min</p>
        </div>
      </div>
    </div>
  );
}

export default ProgressBar;
