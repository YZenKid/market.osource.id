export type RecentOrder = {
  orderNumber: string;
  customerName: string;
  brandLabel: string;
  total: string;
  paymentStatus: string;
  fulfillmentStatus: string;
  timeLabel: string;
  href: string;
};

export type QueueItem = {
  label: string;
  count: number;
  helper: string;
  href: string;
};

export type HealthItem = {
  label: string;
  value: string;
  helper: string;
  tone: 'neutral' | 'primary' | 'success' | 'warning' | 'destructive' | 'secondary';
};
