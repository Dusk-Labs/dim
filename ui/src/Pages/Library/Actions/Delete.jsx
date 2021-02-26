import React, { useCallback } from "react";
import { connect } from "react-redux";
import { useHistory } from "react-router";
import { delLibrary } from "../../../actions/library";
import ConfirmationBox from "../../../Modals/ConfirmationBox";

const Delete = (props) => {
  const history = useHistory();

  const { del_library, auth, delLibrary } = props;

  const removeLib = useCallback(async () => {
    if (del_library.deleting) return;

    await delLibrary(auth.token, props.id);

    // redirect to dashboard when removed
    history.push("/");
  }, [auth.token, delLibrary, del_library.deleting, history, props.id]);

  const { deleting } = props.del_library;

  return (
    <div className="deleteLibraryAction">
      <ConfirmationBox
        contentLabel="removeLib"
        action={removeLib}
        msg="Are you sure you want to remove this library?"
      >
        <button className={`deleting-${deleting}`}>Remove library</button>
      </ConfirmationBox>
    </div>
  )
};

const mapStateToProps = (state) => ({
  auth: state.auth,
  del_library: state.library.del_library
});

const mapActionsToProps = {
  delLibrary
};

export default connect(mapStateToProps, mapActionsToProps)(Delete);
