import "./UserCard.scss";

export default function UserCard(props) {
  return (
    <div className="parent">
      <div className="picture">
        <img alt="profilePicture" src={props.user.image}/>
      </div>
      <span>{props.user.name}</span>
    </div>
  );
}
