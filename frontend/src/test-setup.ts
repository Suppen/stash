import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import "@testing-library/jest-dom";

i18n.use(initReactI18next).init({
    fallbackLng: "nb",
    interpolation: {
        escapeValue: false
    },
    resources: {}
});
