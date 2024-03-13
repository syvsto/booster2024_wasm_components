import { greet as importedGreet } from "greeter:jspkg/greet";

export const greet = {
  greet: function () {
    return importedGreet() + " and Javascript!";
  },
};
