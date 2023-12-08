import { useId, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

export const Frontpage = (): JSX.Element => {
    const componentId = useId();

    const { t } = useTranslation();

    const navigate = useNavigate();

    const [id, setId] = useState<string>("");

    return (
        <div>
            <form
                className="frontpage"
                onSubmit={e => {
                    e.preventDefault();
                    navigate(`/products/${id}`);
                }}
            >
                <div>
                    <label htmlFor={`${componentId}-id`}>{t("product:productId")}</label>
                    <input
                        id={`${componentId}-id`}
                        type="text"
                        value={id}
                        onChange={e => {
                            setId(e.target.value);
                        }}
                        autoFocus
                    />
                </div>
            </form>
        </div>
    );
};
