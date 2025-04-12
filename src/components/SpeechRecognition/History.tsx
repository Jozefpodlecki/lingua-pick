import { History, X, Trash2 } from "lucide-react";
import { useState } from "react";
import { HistoryItem } from "../../models";
import HistoryItemComponent from "./HistoryItemComponent";

interface Props {
    history: HistoryItem[];
    onClear: () => void;
}

const RecognitionHistory: React.FC<Props> = ({ history, onClear }) => {
    const [isOpen, setIsOpen] = useState<boolean>(false);

    return (
        <>
            {!isOpen && (
                <button
                    onClick={() => setIsOpen(true)}
                    className="absolute top-16 left-4 bg-gray-700 text-white p-2 rounded hover:bg-gray-600 z-50 cursor-pointer"
                >
                    <History className="w-5 h-5" />
                </button>
            )}
            <div
                className={`absolute top-0 left-0 h-full bg-gray-800 text-white p-4 shadow-lg transform ${
                    isOpen ? "translate-x-0" : "-translate-x-full"
                } transition-transform duration-300 z-40`}
                style={{ width: "300px" }}
            >
                <div className="flex items-center justify-between mb-4">
                    <div className="flex items-center">
                        <History className="w-5 h-5 mr-2 text-gray-400" />
                        <h2 className="text-xl font-bold">Recognition History</h2>
                    </div>
                    <button onClick={() => setIsOpen(false)} className="text-gray-400 hover:text-white cursor-pointer">
                        <X className="w-5 h-5" />
                    </button>
                </div>
                {history.length > 0 ? (
                    <>
                        <button
                            onClick={onClear}
                            className="flex items-center justify-center text-gray-400 hover:text-white mb-4 cursor-pointer"
                        >
                            <Trash2 className="w-5 h-5" />
                        </button>
                        <ul className="space-y-2 mb-4 overflow-y-auto" style={{ maxHeight: "calc(100vh - 150px)" }}>
                            {history.map((item, index) => (
                                <HistoryItemComponent key={index} item={item} />
                            ))}
                        </ul>
                    </>
                ) : (
                    <p className="text-gray-400">No history yet.</p>
                )}
            </div>
        </>
    );
};

export default RecognitionHistory;