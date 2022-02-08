interface Props {
  content: string;
  max: number;
}

export const TruncText = (props: Props) => {
  const words = props.content.split(" ");

  if (words.length < props.max) {
    return <>{words.join(" ")}</>;
  }

  words.length = props.max;

  return <>{words.join(" ") + "..."}</>;
};

export default TruncText;
