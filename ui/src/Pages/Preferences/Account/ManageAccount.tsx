import DelAccountBtn from "./DelAccountBtn";

const ManageAccount = () => (
  <section>
    <h2>Manage account</h2>
    <p className="desc">
      Your actual media on the system does not get deleted.
    </p>
    <div className="options">
      <DelAccountBtn />
    </div>
  </section>
);

export default ManageAccount;
