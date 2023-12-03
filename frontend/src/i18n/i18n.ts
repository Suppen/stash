import i18next from "i18next";
import { initReactI18next } from "react-i18next";

try {
    await i18next.use(initReactI18next).init({
        fallbackLng: "nb",
        interpolation: {
            escapeValue: false
        },
        resources: {
            nb: {
                translation: {
                    save: "Lagre"
                },
                stashItem: {
                    quantity: "Antall",
                    expiryDate: "Utløpsdato",
                    quantityMustBeGreaterThanZero: "Antall må være større enn 0",
                    quantityIsRequired: "Antall er påkrevd",
                    expiryDateIsRequired: "Utløpsdato er påkrevd"
                }
            }
        }
    });
} catch (e) {
    console.error("Failed to initialize i18next");
    console.error(e);
}
