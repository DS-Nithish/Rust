import React from "react";

function Greeting(props) {
    return <h1>Hello, {props.name}!</h1>;
}

function Welcome() {
    return <h1>You are Welcome!!!!!</h1>;
}

function Bye(props) {
    return <h1>See you, {props.name}!</h1>;
}
export { Greeting, Welcome, Bye };
