import Toggle from "../../../Components/Toggle";

function Cards() {
  return (
    <section>
      <h2>Cards</h2>
      <Toggle
        state={true}
        name="Show media name under cards across the dashboard and libraries"
      />
    </section>
  );
}

export default Cards;
