import UserCard from "../../Components/CardList/UserCard";
import Invites from "./Invites";
import MyAccount from "./MyAccount";

function Account() {
  /*
      <section className="usersSection">
        <div className="sectionHeading">
          <span>Users</span>
          <button className="editBtn">
            edit
          </button>
        </div>
        <div className="userListContainer">
          <UserCard user={{"name": "placeholder", "image": "https://i.redd.it/3n1if40vxxv31.png"}}/>
          <UserCard user={{"name": "placeholder2", "image": "https://i.redd.it/3n1if40vxxv31.png"}}/>
        </div>
      </section>
  */
  return (
    <>
      <MyAccount/>
      <Invites/>
    </>
  );
}

export default Account;
