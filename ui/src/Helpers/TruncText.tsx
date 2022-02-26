interface Props {
  content: string;
  max: number;
}

export const TruncText = (props: Props) => {
  const words = props.content.split(" ");

  if (words.length <= props.max) {
    return <>{words.join(" ")}</>;
  }

  const sliced = words.slice(0, -(words.length - props.max));

  return <>{sliced.join(" ") + "..."}</>;
};

export default TruncText;
