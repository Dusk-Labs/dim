import { useEffect, useRef, useState } from "react";

function GhostCards() {
  const sectionRef = useRef<HTMLElement | null>(null);

  const [count, setCount] = useState(0);

  useEffect(() => {
    if (!sectionRef.current) return;

    setCount(Math.floor(sectionRef.current.offsetWidth / 230) * 2);
  }, []);

  const cards = [];

  for (let x = 0; x < count; x++) {
    cards.push(
      <div key={x} className="card-wrapper">
        <div className="card">
          <div className="placeholder" />
        </div>
      </div>
    );
  }

  return (
    <section className="showAfter100ms" ref={sectionRef}>
      <div className="placeholderText" />
      <div className="cards">{cards}</div>
    </section>
  );
}

export default GhostCards;
