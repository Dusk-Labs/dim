import Toggle from "../../../Components/Toggle";

function Sidebar() {
  return (
    <section>
      <h2>Sidebar</h2>
      <Toggle
        name="Keep the sidebar always in compact mode"
      />
    </section>
  );
}

export default Sidebar;
