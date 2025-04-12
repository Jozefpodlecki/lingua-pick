import React from "react";
import { HistoryItem } from "../../models";
import { formatDistanceToNow } from "date-fns";

interface Props {
    item: HistoryItem;
}

const HistoryItemComponent: React.FC<Props> = ({ item }) => {
    return (
        <li className="bg-gray-700 p-2 rounded">
            <div className="flex justify-between">
                <span>{item.word}</span>
                <span className="text-gray-400 text-sm">
                    {formatDistanceToNow(new Date(item.createdOn), { addSuffix: true })}
                </span>
            </div>
        </li>
    );
};

export default HistoryItemComponent;