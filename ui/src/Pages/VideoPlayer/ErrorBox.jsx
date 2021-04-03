function ErrorBox(props) {
  const { error, setError, currentTime } = props;

  const reloadPlayer = () => {
    sessionStorage.setItem("currentTime", currentTime);
    window.location.reload();
  };

  console.log(error)

  return (
    <div className="errorBox">
      <h2>Error</h2>
      <div className="separator"/>
      <p>{error.msg}</p>
      {error.errors.map((err, i) => (
        <details key={i}>
          <summary>({++i})</summary>
          <div className="stderr">
            <code>{err}</code>
          </div>
        </details>
      ))}
      <div className="options">
        <button onClick={() => setError(false)}>Hide</button>
        <button onClick={reloadPlayer}>Retry</button>
      </div>
    </div>
  );
}

export default ErrorBox;
