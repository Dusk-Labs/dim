import "./Ring.scss";

type RingProps = {
  small: boolean;
};

function RingLoad(props: RingProps) {
  return (
    <div className={`ringLoad small-${props.small}`}>
      <div />
    </div>
  );
}

export default RingLoad;
