import("../pkg/index.js").catch(console.error);

document.onkeydown = function (e) {
    var input = document.getElementById("input");
    var output = document.getElementById("command_output");

    var charCode = (typeof e.which == "number") ? e.which : e.keyCode;

    if (charCode) {

        switch (charCode) {
            case 8:
                input.innerText = input.innerText.substring(0, input.innerText.length - 1);

                return;
            case 13:
                if (input.innerText == "") {
                    return;
                }

                input.innerText = "";
                output.innerText = "[" + Date.now() + "] unimplemented";

                return;
        }

        input.innerText += String.fromCharCode(charCode);
    }
};
