import i18next from "i18next";
import { initReactI18next } from "react-i18next";

await i18next.use(initReactI18next).init({
    fallbackLng: "nb",
    interpolation: {
        escapeValue: false
    },
    resources: {
        nb: {
            translation: {
                save: "Lagre",
                findProduct: "Finn produkt",
                loading: "Laster",
                unknownError: "Ukjent feil",
                actions: "Handlinger"
            },
            stashItem: {
                quantity: "Antall",
                expiryDate: "Utløpsdato",
                quantityMustBeGreaterThanZero: "Antall må være større enn 0",
                quantityIsRequired: "Antall er påkrevd",
                expiryDateIsRequired: "Utløpsdato er påkrevd",
                expiryDateIsInvalid: "Utløpsdato er ugyldig",
                quantityIsInvalid: "Antall er ugyldig",
                add: "Legg til",
                noStashItems: "Ingen varer i beholdningen"
            },
            product: {
                id: "ID (strekkode)",
                productId: "Produkt-ID (strekkode)",
                brand: "Merke",
                name: "Navn",
                idIsRequired: "ID strekkode er påkrevd",
                brandIsRequired: "Merke er påkrevd",
                nameIsRequired: "Navn er påkrevd",
                couldNotFindProduct: "Fant ikke produktet",
                idIsInvalid: "ID er ugyldig",
                brandIsInvalid: "Merke er ugyldig",
                newProduct: "Nytt produkt",
                updateProduct: "Oppdater produkt"
            }
        }
    }
});
