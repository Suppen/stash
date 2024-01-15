import { useState } from "react";
import ProductId from "../../domain/valueObjects/ProductId";
import { TextField } from "@mui/material";
import { useTranslation } from "react-i18next";

export type Props = {
    onScan: (value: ProductId) => void;
};

export const IdScanField = ({ onScan }: Props) => {
    const { t } = useTranslation();

    const [value, setValue] = useState<string>("");

    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === "Enter") {
            onScan(new ProductId(value));
            setValue("");
        }
    };

    return (
        <TextField
            variant="outlined"
            fullWidth
            label={t("product:productId")}
            value={value}
            onChange={e => {
                setValue(e.target.value);
            }}
            onKeyDown={handleKeyDown}
            autoFocus
        />
    );
};
