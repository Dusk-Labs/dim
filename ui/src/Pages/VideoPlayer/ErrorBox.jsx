function ErrorBox(props) {
  const { error, setError, currentTime } = props;

  const reloadPlayer = () => {
    sessionStorage.setItem("currentTime", currentTime);
    window.location.reload();
  };

  return (
    <div className="errorBox">
      <h2>Error</h2>
      <div className="separator"/>
      <p>{error?.message}</p>
      <div className="options">
        <button onClick={() => setError(false)}>Hide</button>
        <button onClick={reloadPlayer}>Retry</button>
      </div>
    </div>
  );
}

export default ErrorBox;
