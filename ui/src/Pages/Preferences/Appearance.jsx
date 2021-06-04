function ThemeContainer({options}) {
  return (
    <div className="themeParent">
      <div className="themeImage" style={{backgroundImage: `url('${options.img}')`}}>
      </div>
      <div className="themeInfo">
        <span>{options.name}</span>
      </div>
    </div>
  );
}

function Appearance() {
  return (
    <>
      <section className="appearanceSection">
        <div className="sectionHeading">
          <span>Theme</span>
        </div>
        <div className="themeContainer">
          <ThemeContainer
            options={{
              name: "Dark default",
              img: "https://media.discordapp.net/attachments/713416964802084955/850373391612903424/Component_2.png?width=594&height=308"
            }}
          />
          <ThemeContainer
            options={{
              name: "Light",
              img: "https://media.discordapp.net/attachments/834495310332035126/850379843153821726/Component_3.png?width=594&height=308"
            }}
          />
        </div>
      </section>
      <section className="uiSection">
        <div className="sectionHeading">
          <span>Interface</span>
        </div>
        <div className="uiContainer">

        </div>
      </section>
    </>
  );
}

export default Appearance;
