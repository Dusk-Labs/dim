function Account() {
  return (
    <>
      <section className="accountSection">
        <div/>
        <div/>
        <div/>
      </section>
      <section className="usersSection">
        <div className="sectionHeading">
          <span>Users</span>
          <button className="editBtn">
            edit
          </button>
        </div>
      </section>
      <section className="tokenSection">
        <div className="sectionHeading">
          <span>Tokens</span>
          <button className="editBtn">
            add
          </button>
        </div>
      </section>
    </>
  );
}

export default Account;
