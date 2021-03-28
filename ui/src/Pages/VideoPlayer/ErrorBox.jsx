function ErrorBox(props) {
  const { error, setError } = props;

  return (
    <div className="errorBox">
      <h2>Error</h2>
      <div className="separator"/>
      <p>{error?.message}</p>
      <div className="options">
        <button onClick={() => setError(false)}>Hide</button>
        <button onClick={() => window.location.reload()}>Reload player</button>
      </div>
    </div>
  );
}

export default ErrorBox;
