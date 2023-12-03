export type Props = {
    children?: string;
};

export const ErrorMessage = ({ children }: Props): JSX.Element | null =>
    children === undefined ? null : <p className="error-message">{children}</p>;
