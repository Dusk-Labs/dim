import "./Crumbs.scss";

function Crumbs(props) {
  const crumbs = [];

  for (let x = 0; x < props.count; x++) {
    crumbs.push(
      <span
        className={`crumb ${props.activeIndex === x ? "active" : "hidden"}`}
        key={x}
        data-key={x}
        onClick={props.toggle}
      />
    );
  }

  return <div className="bannerCrumbs">{crumbs}</div>;
}

export default Crumbs;
