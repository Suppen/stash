/** A date string in the format YYYY-MM-DD, guaranteed to be a valid date */
export class PlainDate {
    /** Date string in the format YYYY-MM-DD, guaranteed to be a valid date */
    #dateStr: string;

    constructor(dateStr: string) {
        // Make sure the string is on the format YYYY-MM-DD
        if (!/^\d{4}-\d{2}-\d{2}$/.test(dateStr)) {
            throw new Error(`Invalid date string: ${dateStr}`);
        }

        // Make sure the date is valid
        const date = new Date(dateStr);

        // XXX Apparently Chrome is way more permissive than Firefox. FF will say that 2021-02-31 is invalid, while
        // Chrome will say that it's 2021-03-03. This means we can't do the normal `Number.isNaN(date.getTime())` here.
        // Instead, check if the ISO string is the same as the input string. If it is, the date is valid.
        if (date.toISOString().slice(0, 10) !== dateStr) {
            throw new Error(`Invalid date string: ${dateStr}`);
        }

        // Save it
        this.#dateStr = dateStr;
    }

    /**
     * Creates a PlainDate from a Date object
     *
     * @param date The date to create a PlainDate from
     *
     * @returns A PlainDate
     */
    static fromDate(date: Date) {
        return new PlainDate(date.toISOString().slice(0, 10));
    }

    /** Returns the date string in the format YYYY-MM-DD */
    toString() {
        return this.#dateStr;
    }

    /** Returns the date as an ISO 8601 date string */
    toISOString() {
        return this.#dateStr;
    }
}

export default PlainDate;
